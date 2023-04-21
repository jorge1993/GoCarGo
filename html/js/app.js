//gsap.to('.Ojos',{duration: 1, x: 100, opacity: 0, delay: 1});

// fucnicion para que el ojo se mueva
function moverOjo(){
    //gsap.to('.Ojos',{duration: 1, x: 100, opacity: 0, delay: 1});
    gsap.to(".Ojos", {
        motionPath: {
            path: "#path",
            align: "#path",
            alignOrigin: [0.5, 0.5],
            autoRotate: true
        },
        duration: 5,
        ease: "power1.inOut"
    });
}

// metodo post para pedir estado y preguntar contantemente y en paralelo
function state(){
    $.post("http://localhost:8000/state", function(data){
        console.log(data);
        if(data == 1){
            moverOjo();
        }
    });
}
    
// metodo click de un boton
$("#btn").click(moverOjo());

// funcion para que se ejecute cada 1 segundo
//setInterval(state(), 1000);
