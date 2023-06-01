import React, { useState } from 'react';
import axios from 'axios';
import "../css/LoginForm.css";

const LoginForm = ({handleLogin}) => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const submitLogin = e => {
    e.preventDefault();
    axios.post("http://localhost:8000/v1/auth/login",
      {
        username,
        password,
      }, {
        headers: {"Access-Control-Allow-Origin": "*"}
      })
      .then(response => {
        setUsername("");
        setPassword("");
        console.log(JSON.stringify(response.data));
        handleLogin(response.data["token"]);
      })
      .catch(error => {
        alert(error);
        setUsername("");
        setPassword("");
      })
  };
  const handlePasswordChange = e => setPassword(e.target.value);
  const handleUsernameChange = e => setUsername(e.target.value);

  return (
    <form className="login" onSubmit={submitLogin}>
      <h1 className="login-title">Login</h1>
      <input 
        type="text" 
        className="login-input"
        placeholder="Username"
        autoFocus
        onChange={handleUsernameChange}
        value={username} />
      <input 
        type="password"
        className="login-input"
        placeholder="Password"
        onChange={handlePasswordChange}
        value={password} />
      <input type="submit" value="Let's Go" className="login-button" />
    </form>
  )
}

export default LoginForm;