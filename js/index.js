import("../pkg/index.js").then(module => {
    module.run();
    let chartlet = module.Chartlet.new("chartlet");
    chartlet.run();
}).catch(console.error);
