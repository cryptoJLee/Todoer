import React, { useState } from "react";
import axios from "axios";
import "../App.css";

function ToDoItem({title, status, passBackResponse}) {
  const processStatus = status => status === "PENDING" ? "edit" : "delete";
  const button = processStatus(status);
  const inverseStatus = status => status === "PENDING" ? "DONE" : "PENDING";
  const sendRequest = () => {
    axios.post(
      "http://127.0.0.1:8000/v1/item/" + button, 
      {
        title,
        status: inverseStatus(status)
      }, 
      {
        headers: {"token": "some_token"}
      }
    ).then(response => passBackResponse(response));
  };

  return (
    <div className="itemContainer">
      <p>{title}</p>
      <div className="actionButton" onClick={sendRequest}>{button}</div>
    </div>
  );
}

export default ToDoItem;