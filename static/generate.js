function generate() {
    const Http = new XMLHttpRequest();
    const url='./generate_new_one';
    Http.open("POST", url);
    Http.send();
    Http.onreadystatechange=(e)=>{
        document.getElementById("display_random_data").innerHTML = Http.responseText
    }
}
