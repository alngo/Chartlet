import("../pkg/index.js").then(module => {
    let chartlet = module.Chartlet.new();
    let bridge = module.HtmlBridge.new("chartlet");
    chartlet.connect_to(bridge);
    chartlet.run();
}).catch(console.error);
