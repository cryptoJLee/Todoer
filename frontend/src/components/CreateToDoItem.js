import React, { useState } from "react";
import axios from "axios";
import "../App.css";

const CreateToDoItem = ({passBackResponse}) => {
  const [title, setTitle] = useState("");
  const createItem = () => {
    axios.post("http://127.0.0.1:8000/v1/item/create/" + title,
      {},
      {headers: {"token": "some_token"}}
    ).then(response => {
      setTitle("");
      passBackResponse(response);
    })
  };
  const handleTitleChange = e => setTitle(e.target.value);

  return (
    <div className="inputContainer">
      <input type="text" id="name"
            placeholder="create to do item"
            value={title}
            onChange={handleTitleChange} />
      <div className="actionButton"
            id="create-button"
            onClick={createItem}>
        Create
      </div>
    </div>
  );
}

export default CreateToDoItem;