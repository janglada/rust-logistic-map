import("../pkg/index.js")
    .then(r => {
        let STEPS =  1000;
        let a = r.run(STEPS);
        let canvas = document.querySelector("#main")
        let w = canvas.width;
        let h = canvas.height;
        let canvas2 = document.querySelector("#freq")
        let w2 = canvas.width;
        let h2 = canvas.height;

        let context = canvas.getContext("2d");
        let context2= canvas2.getContext("2d");

        context.fillStyle = "#000000";
        context.beginPath();
        context.moveTo(0,h);
        context.lineTo(w, h);
        context.stroke();
        context.beginPath();
        context.moveTo(0,0);
        context.lineTo(0, h);
        context.stroke();

        context2.fillStyle = "#000000";
        context2.beginPath();
        context2.moveTo(0,h2);
        context2.lineTo(w2, h2);
        context2.stroke();
        context2.beginPath();
        context2.moveTo(0,0);
        context2.lineTo(0, h2);
        context2.stroke();

        for (let i = 0; i < a.length; i++) {
            let x = i*(1/STEPS)*w/4;

            let unique = new Set(a[i])
            unique.forEach(v => {
                let y = h - v*h;
                context.fillRect(x, y, 1, 1);
            });

            let y2 = h - (unique.size/256)*h2;
            context2.fillRect(x, y2, 1, 1);
        }


    })
    .catch(console.error);
