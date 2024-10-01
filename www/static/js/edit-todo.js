document.addEventListener("DOMContentLoaded", (event) => {
  let titleElement = document.querySelector("[data-js='title']");
  let bodyElement = document.querySelector("[data-js='body']");
  let bottonElement = document.querySelector("[data-js='edit-btn']");

  let title = titleElement.value;
  let body = bodyElement.value;

  console.log({ title, body });

  //  TODO: add logic here

  if (!titleElement) {
    throw new Error("Input not found!");
  }

  if (!bodyElement) {
    throw new Error("Textarea not found!");
  }

  if (!bottonElement) {
    throw new Error("Botton not found!");
  }
});
