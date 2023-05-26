
import { useEffect, useState } from 'react';
import './App.css';
import axios from 'axios';
import ToDoItem from './components/TodoItem';
import CreateToDoItem from './components/CreateToDoItem';

function App() {
  const [pendingItems, setPendingItems] = useState([]);
  const [doneItems, setDoneItems] = useState([]);
  const handleReturnedState = (response) => {
    let pendingItems = response.data["pending_items"];
    let doneItems = response.data["done_items"];
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
        />
      );
    })
    return itemList;
  }  
  useEffect(() => {
    axios.get(
      "http://127.0.0.1:8000/v1/item/get", 
      {headers: {"token": "some_token"}}
    ).then(response => {
      setPendingItems(processItemValues(response.data["pending_items"]));
      setDoneItems(processItemValues(response.data["done_items"]));
    });
  }, []);
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
}

export default App;
