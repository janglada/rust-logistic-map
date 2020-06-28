import("../pkg/index.js")
    .then(r => {
        let STEPS =  1000;
        let a = r.run(STEPS);
        let canvas = document.querySelector("canvas")
        let w = canvas.width;
        let h = canvas.height;

        let context = canvas.getContext("2d");
        context.fillStyle = "#000000";
        for (let i = 0; i < a.length; i++) {
            let x = i*(1/STEPS)*w/4;

            new Set(a[i]).forEach(v => {
                let y = h - v*h;
                context.fillRect(x, y, 1, 1);
            });
            // for (let j = 0; j < a[i].length; j++) {
            //
            // }
            // console.log({x,y})

        }


    })
    .catch(console.error);
