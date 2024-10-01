document.addEventListener("DOMContentLoaded", (event) => {
  const delete_btns = document.querySelectorAll("[data-js^='delete']");
  const complete_btns = document.querySelectorAll("[data-js^='complete']");

  const handleTodoOn = (method, e) => {
    e.preventDefault();

    const todo_id = e.target.dataset.js.split("_")[1];
    const url = `/api/todos/${
      method == "PATCH" ? "complete" : "delete"
    }/${todo_id}`;

    fetch(url, { method })
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
    btn.addEventListener("click", (e) => {
      handleTodoOn("DELETE", e);
    });
  });

  complete_btns.forEach((btn) => {
    btn.addEventListener("click", (e) => {
      handleTodoOn("PATCH", e);
    });
  });
});
