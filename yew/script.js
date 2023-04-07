document.body.style.cursor = "default";

setTimeout(function() {
  const resizer = document.querySelector(".resizer");
  const sidebar = document.querySelector(".sidebars");

  resizer.addEventListener("mousedown", (event) => {
    document.body.style.cursor = "col-resize";
    document.addEventListener("mousemove", resize, false);

    document.addEventListener("mouseup", () => {
      document.removeEventListener("mousemove", resize, false);
      document.body.style.cursor = 'default';
    }, false);
  });

  function resize(e) {
    const size = `${e.x}px`;
    sidebar.style.width = size;
  }

  sidebar.style.width = '250px';
}, 5000);

setTimeout(function() {
  const resizer2 = document.querySelector(".resizer2");
  const req = document.querySelector(".req");

  resizer2.addEventListener("mousedown", (event) => {
    document.body.style.cursor = "row-resize";
    document.addEventListener("mousemove", resize, false);

    document.addEventListener("mouseup", () => {
      document.removeEventListener("mousemove", resize, false);
      document.body.style.cursor = 'default';
    }, false);
  });

  function resize(e) {
    const size = `${e.y}px`;
    req.style.height = size;
  }

  req.style.height = '325px';
}, 5000);
