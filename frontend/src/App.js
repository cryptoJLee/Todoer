
import { useEffect, useState } from 'react';
import './App.css';
import axios from 'axios';
import ToDoItem from './components/TodoItem';
import CreateToDoItem from './components/CreateToDoItem';
import LoginForm from './components/LoginForm';

function App() {
  const [pendingItems, setPendingItems] = useState([]);
  const [doneItems, setDoneItems] = useState([]);
  const [loginStatus, setLoginStatus] = useState(false);

  const handleReturnedState = (response) => {
    let pendingItems = response.data["pending_items"];
    let doneItems = response.data["done_items"];
    localStorage.setItem("item-cache-date", new Date());
    localStorage.setItem("item-cache-data-pending", JSON.stringify(pendingItems));
    localStorage.setItem("item-cache-data-done", JSON.stringify(doneItems));
    setPendingItems(processItemValues(pendingItems));
    setDoneItems(processItemValues(doneItems));
  }
  function processItemValues(items) {
    let itemList = [];
    items.forEach((item, index) => {
      itemList.push(
        <ToDoItem key={item.title + item.status}
                  title={item.title}
                  status={item.status}
                  passBackResponse={handleReturnedState}
                  logout={logout}
        />
      );
    })
    return itemList;
  }
  function logout() {
    localStorage.removeItem("token");
    setLoginStatus(false);
  }
  function getItems() {
    let cachedData = Date.parse(localStorage.getItem("item-cache-date"));
    let now = new Date();
    let difference = Math.round((now - cachedData) / 1000);
    if (difference <= 120) {
      let pendingItems = JSON.parse(localStorage.getItem("item-cache-data-pending"));
      let doneItems = JSON.parse(localStorage.getItem("item-cache-data-done"));
      setPendingItems(processItemValues(pendingItems));
      setDoneItems(processItemValues(doneItems));
    } else {
      axios.get(
        "http://127.0.0.1:8000/v1/item/get", 
        {
          headers: {"token": localStorage.getItem("user-token")}
        }
      ).then(response => {
        let pending_items = response.data["pending_items"];
        let done_items = response.data["done_items"];
        localStorage.setItem("item-cache-date", new Date());
        localStorage.setItem("item-cache-data-pending", JSON.stringify(pending_items));
        localStorage.setItem("item-cache-data-done", JSON.stringify(done_items));
        setPendingItems(processItemValues(pending_items));
        setDoneItems(processItemValues(done_items));
      }).catch(error => {
        if (error.response.status === 401) logout();
      });
    }
  }
  const handleLogin = token => {
    localStorage.setItem("user-token", token);
    setLoginStatus(true);
    getItems();
  }
  useEffect(() => {
    let token = localStorage.getItem("user-token");
    if (token !== null) {
      setLoginStatus(true);
      getItems();
    }
    return () => logout();
  }, []);
  if (loginStatus) {
    return (
      <div className="App">
        <div className="mainContainer">
          <h1>Done Items</h1>
          <p>done item count: {doneItems.length}</p>
          {doneItems}
          <h1>Pending Items</h1>
          <p>pending item count: {pendingItems.length}</p>
          {pendingItems}
          <CreateToDoItem passBackResponse={handleReturnedState} />
        </div>
      </div>
    );
  } else {
    return (
      <div className="App">
        <div className="mainContainer">
          <LoginForm handleLogin={handleLogin} />
        </div>
      </div>
    )
  }
}

export default App;
