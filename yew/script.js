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
    sidebar.style.flexBasis = size;
  }

  sidebar.style.flexBasis = '325px';
}, 5000);

setTimeout(function() {
  const resizer2 = document.querySelector(".resizer2");
  const req = document.querySelector(".req");

  resizer2.addEventListener("mousedown", (event) => {
    document.body.style.cursor = "col-resize";
    document.addEventListener("mousemove", resize, false);

    document.addEventListener("mouseup", () => {
      document.removeEventListener("mousemove", resize, false);
      document.body.style.cursor = 'default';
    }, false);
  });

  function resize(e) {
    const size = `${e.y}px`;
    req.style.flexBasis = size;
  }

  req.style.flexBasis = '325px';
}, 5000);
