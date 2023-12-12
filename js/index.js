import("../pkg/index.js").then(module => {
    let root = document.getElementById("chartlet");
    let chartlet = module.App.new(root);
    chartlet.run();
}).catch(console.error);
