document.addEventListener("DOMContentLoaded", () => {
  const msg_boxes = document.querySelectorAll("[data-js='msg']");

  const handleClickMessage = (e) => {
    let classes = e.target.parentElement.classList;
    classes.remove("anim-come-in");
    classes.add("anim-go-away");
  };

  msg_boxes.forEach((msg_box) => {
    msg_box.addEventListener("click", handleClickMessage);

    setTimeout(() => {
      msg_box.classList.remove("anim-come-in");
      msg_box.classList.add("anim-go-away");
    }, 4000);
  });
});
