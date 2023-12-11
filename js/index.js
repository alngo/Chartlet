import("../pkg/index.js").then(module => {
    let root = document.getElementById("chartlet");
    let chartlet = module.App.new(root);

    window.addEventListener("resize", (e) => {
        chartlet.resize(root);
    });

    chartlet.run();


}).catch(console.error);
