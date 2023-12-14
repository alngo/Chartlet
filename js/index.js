import("../pkg/index.js").then(module => {
    let root = document.getElementById("chartlet");
    // let width = root.clientWidth;
    // let height = root.clientHeight;
    let app = module.App.new(width, height);
    // Add 10 random open high low close bars
    //app.add_ohlc()

    // app.set_shift(10);
    // let chartlet = module.OldApp.new(root);
    // chartlet.run();
}).catch(console.error);
