import("../pkg/index.js").then(module => {
    let charlet = window.document.getElementById("chartlet");
    module.run(chartlet.clientWidth, chartlet.clientHeight);
}).catch(console.error);
