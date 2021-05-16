import _axios from "axios";
import { useEffect, useState, ChangeEvent } from "react";

export default function Index() {
  const [todos, setTodos] = useState([]);
  const [input, setInput] = useState("");

  const axios = _axios.create({
    baseURL: "http://localhost:8080/",
    timeout: 1000,
  });

  const asyncFunc = async (fn) => {
    await fn();
    await getAllTodos();
  };

  const getAllTodos = async () => {
    const todos = await axios.get("todos").then((res) => {
      const todo = res.data;
      return todo;
    });
    setTodos(todos);
  };

  useEffect(() => {
    getAllTodos();
  }, []);

  const addTodo = () => {
    asyncFunc(async () => {
      await axios.post("todos", {
        name: input,
        status: "to_do",
      });
    });
    setInput("");
  };

  const deleteTodo = (todoId: string) => {
    asyncFunc(async () => {
      await axios.delete(`todos/${todoId}`);
    });
  };

  const onInputChange = (ev: ChangeEvent<HTMLInputElement>) => {
    setInput(ev.target.value);
  };

  return (
    <div>
      <input value={input} onChange={onInputChange} />
      <button onClick={addTodo}>ADD TODO</button>
      <h1>TODO</h1>
      <ul>
        {0 < todos.length &&
          todos.flatMap((todo) => {
            return (
              <li key={todo.id}>
                <p>{todo.name}</p>
                <p>{todo.status}</p>
                <button onClick={() => deleteTodo(todo.id)}>DELETE</button>
              </li>
            );
          })}
      </ul>
    </div>
  );
}
