import React, { useState } from "react";
import axios from "axios";
import "../App.css";

function ToDoItem({title, status, passBackResponse, logout}) {
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
        headers: {"token": localStorage.getItem("user-token")}
      }
    ).then(response => passBackResponse(response)
    ).catch(error => {
      if (error.response.status === 401) logout();
    });
  };

  return (
    <div className="itemContainer">
      <p>{title}</p>
      <div className="actionButton" onClick={sendRequest}>{button}</div>
    </div>
  );
}

export default ToDoItem;