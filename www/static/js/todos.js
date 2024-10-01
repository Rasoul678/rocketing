document.addEventListener("DOMContentLoaded", (event) => {
  const delete_btns = document.querySelectorAll("[data-js^='delete_']");
  const complete_btns = document.querySelectorAll("[data-js^='complete_']");

  const handleDeleteTodo = (e) => {
    event.preventDefault();
    const todo_id = e.target.dataset.js.split("_")[1];
    const url = `/api/todos/delete/${todo_id}`;

    fetch(url, {
      method: "DELETE",
    })
      .then((response) => {
        if (response.ok) {
          window.location.reload();
        }
      })
      .catch((error) => {
        console.log(error);
      });
  };

  delete_btns.forEach((btn) => {
    btn.addEventListener("click", handleDeleteTodo);
  });

  const handleCompleteTodo = (e) => {
    const todo_id = e.target.dataset.js.split("_")[1];
    const url = `/api/todos/complete/${todo_id}`;

    fetch(url, {
      method: "PATCH",
    })
      .then((response) => {
        if (response.ok) {
          window.location.reload();
        }
      })
      .catch((error) => {
        console.log(error);
      });
  };

  complete_btns.forEach((btn) => {
    btn.addEventListener("click", handleCompleteTodo);
  });
});
