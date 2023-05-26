# Rust Web App for To-Do List Management

This is a web-based application written in Rust which allows you to manage your to-do list, with a backend built on the Actix framework and a frontend using React. This README file details how to get started using the application.

## Getting Started

To run this application, you will need to have Rust and Node.js installed on your computer.

### Backend

1. Start by cloning the repository to your local machine.

```
git clone [repository_url]
```

2. Navigate to the `backend` directory of the project.

```
cd rust-todo-app/backend
```

3. Install the dependencies

```
cargo build
```

4. Run the server

```
cargo run
```

The server should now be running on `http://localhost:8000`.

### Frontend

1. Navigate to the `frontend` directory of the project.

```
cd rust-todo-app/frontend
```

2. Install the dependencies

```
npm install
```

3. Run the server

```
npm start
```

The frontend should now be running on `http://localhost:3000`.

## Usage

Once the server and frontend are running, visit `http://localhost:3000` in your browser to use the application.

You can add new tasks to your to-do list by entering the task description and clicking the "Add Task" button. You can also mark tasks as completed by clicking the checkbox next to the task.

Completed tasks can be removed from the list by clicking the "Clear Completed Tasks" button.

## Conclusion

This is just a basic to-do list management application. You can add more functionality or customize it to suit your needs. If you find any issues or have any questions, feel free to raise an issue in the GitHub repository. 

### Thanks

Thanks for taking the time to read this README file and use my application.