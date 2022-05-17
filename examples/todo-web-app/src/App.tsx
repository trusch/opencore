import React from 'react';
import './App.css';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListItemIcon from '@material-ui/core/ListItemIcon';
import ListItemSecondaryAction from '@material-ui/core/ListItemSecondaryAction';
import ListItemText from '@material-ui/core/ListItemText';
import ListSubheader from '@material-ui/core/ListSubheader';
import Checkbox from '@material-ui/core/Checkbox';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import TextField from '@material-ui/core/TextField';
import Typography from '@material-ui/core/Typography';
import IconButton from '@material-ui/core/IconButton';
import MenuIcon from '@material-ui/icons/Menu';
import DeleteIcon from '@material-ui/icons/Delete';
import Button from '@material-ui/core/Button';
import moment  from 'moment';

import { LoginRequest, RefreshRequest } from "./idp_pb";
import { DeleteResourceRequest, ListResourcesRequest, Resource, UpdateResourceRequest, CreateResourceRequest } from "./catalog_pb";
import { AuthenticationPromiseClient } from "./idp_grpc_web_pb";
import { ResourcesPromiseClient } from "./catalog_grpc_web_pb";
import { Error } from "grpc-web";

declare global {
  interface Window {
    __GRPCWEB_DEVTOOLS__: any; // 👈️ turn off type checking
  }
}

type Todo = {
    id: string;
    subject: string;
    description: string;
    status: string;
    createdAt: string;
}

class App extends React.Component {
  authClient: AuthenticationPromiseClient;
  resourceClient: ResourcesPromiseClient;
  accessToken: string;
  refreshToken: string;
  subjectInputRef: React.RefObject<HTMLInputElement>;
  descriptionInputRef: React.RefObject<HTMLInputElement>;
  state = {
    todos: new Array<Todo>(),
  };

  constructor(props: any) {
    super(props);
    this.authClient = new AuthenticationPromiseClient("http://127.0.0.1:3001", null, null);
    this.resourceClient = new ResourcesPromiseClient("http://127.0.0.1:3001", null, null);
    this.accessToken = "";
    this.refreshToken = "";
    this.subjectInputRef = React.createRef();
    this.descriptionInputRef = React.createRef();
    
    // enable grpc-web dev tools if available
    const enableDevTools = window.__GRPCWEB_DEVTOOLS__ || (() => {});
    enableDevTools([
      this.authClient,
      this.resourceClient,
    ]);
  }

  componentDidMount() {
    this.login().then(this.listTodos);
  }

  login = async (): Promise<void> => {
    const req = new LoginRequest();
    req.setEmail("alice@localhost");
    req.setPassword("password");
    const loginResponse = await this.authClient.login(req, {});
    this.accessToken = loginResponse.getAccessToken();
    this.refreshToken = loginResponse.getRefreshToken();
    setInterval(this.refresh, 1000 * 60);
  }

  refresh = async (): Promise<void> => {
    const req = new RefreshRequest();
    req.setRefreshToken(this.refreshToken);
    const refreshResponse = await this.authClient.refresh(req, {});
    this.accessToken = refreshResponse.getAccessToken();
    this.refreshToken = refreshResponse.getRefreshToken();
  }

  setTodoState = (id: string, event: React.ChangeEvent<HTMLInputElement>) => {
    let newStatus = event.target.checked ? "done": "active";
    let req = new UpdateResourceRequest();
    req.setId(id);
    req.setData(JSON.stringify({
      status: newStatus
    }));
    let metadata = { "Authorization": "Bearer " + this.accessToken };
    this.resourceClient.update(req, metadata).then((res: Resource) => {
      this.setState({
        todos: this.state.todos.map((todo: Todo) => {
          if (todo.id === id) {
            console.log(`update status to ${newStatus}`);
            todo.status = newStatus;
          }
          return todo;
        })
      });
    });
  }

  removeTodo = (id: string) => {
    let req = new DeleteResourceRequest();
    req.setId(id);
    let metadata = { "Authorization": "Bearer " + this.accessToken };
    this.resourceClient.delete(req, metadata).then((res: Resource) => {
      this.setState({
        todos: this.state.todos.filter((todo: Todo) => todo.id !== id)
      });
    });
  }

  addTodo = (e: React.FormEvent) => {
    e.preventDefault();
    let subject =  this.subjectInputRef.current!.value;
    let description =  this.descriptionInputRef.current!.value;
    if (!subject) {
      return;
    }
    let req = new CreateResourceRequest();
    req.setKind("todo");
    req.setData(JSON.stringify({
      subject,
      description,
      status: "active",
    }));
    let metadata = { "Authorization": "Bearer " + this.accessToken };
    this.resourceClient.create(req, metadata).then((res: Resource) => {
      let todo: Todo = {
        id: res.getId(),
        subject: subject,
        description: description,
        status: "active",
        createdAt: moment(res.getCreatedAt().toDate()).fromNow(),
      };
      this.setState({
        todos: [todo, ...this.state.todos],
      });
      this.subjectInputRef.current!.value = "";
      this.descriptionInputRef.current!.value = "";
    });
  }

  listTodos = () => {
    const req = new ListResourcesRequest();
    req.setKind("todo");
    let metadata = { "Authorization": "Bearer " + this.accessToken };
    let stream = this.resourceClient.list(req, metadata);
    this.setState({todos: []});

    // listen to the stream and for each todo create an entry in the list
    stream.on("data", (res: Resource) => {
      console.log(res.getId(), JSON.parse(res.getData()));
      let todo = JSON.parse(res.getData());
      todo.id = res.getId();
      todo.createdAt = moment(res.getCreatedAt().toDate()).fromNow();
      this.setState({
        todos: [...this.state.todos, todo],
      });
    });

    // log if an error occurs
    stream.on("error", (err: Error) => {
        console.error(err);
    });
  }

  render() {
    return (
      <div className="App">
        <AppBar position="static">
          <Toolbar>
            <IconButton edge="start"  color="inherit" aria-label="menu">
              <MenuIcon />
            </IconButton>
            <Typography variant="h6" >
              Todo App
            </Typography>
          </Toolbar>
        </AppBar>
        <header>
          <img src="https://cataas.com/cat/cute?height=200" className="App-logo" alt="logo" />
        </header>
        <form onSubmit={(e)=>{this.addTodo(e)}}>
          <TextField id="subject" inputRef={this.subjectInputRef} label="Subject" style={{margin: "1%"}}/>
          <TextField id="description" inputRef={this.descriptionInputRef} label="Description" style={{width: "70%", margin: "1%"}}/>
          <Button type="submit" variant="contained" color="primary" style={{margin: "1%"}}>
            Add Todo
          </Button> 
        </form>
        <List>
        {this.state.todos.map((todo: Todo) => (
          <ListItem key={todo.id} button className="ListItem">
            <ListItemIcon>
              <Checkbox
                edge="start"
                checked={todo.status === "done"}
                tabIndex={-1} 
                disableRipple
                onChange={(event) => this.setTodoState(todo.id, event)}
              />
            </ListItemIcon>
            <ListSubheader>{todo.createdAt}</ListSubheader>
            <ListItemText primary={todo.subject} secondary={todo.description} />
            <ListItemSecondaryAction>
              <IconButton edge="end" aria-label="comments" onClick={()=>{this.removeTodo(todo.id)}}>
                <DeleteIcon />
              </IconButton>
            </ListItemSecondaryAction>
          </ListItem>
        ))}
        </List>
      </div>
    );
  }
}   

export default App;
