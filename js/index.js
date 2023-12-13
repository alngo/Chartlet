import("../pkg/index.js").then(module => {
    let root = document.getElementById("chartlet");
    let width = root.clientWidth;
    let height = root.clientHeight;

    let app = module.App.new(width, height);
    app.set_shift(10);

    let chartlet = module.OldApp.new(root);
    chartlet.run();
}).catch(console.error);
