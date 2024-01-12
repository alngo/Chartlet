import("../pkg/index.js").then(module => {
    let chartlet = module.Chartlet.new();
    chartlet.run();
}).catch(console.error);
