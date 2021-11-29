function httpGetAsync(theUrl, callback)
{
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function() { 
        if (xmlHttp.readyState == 4 && xmlHttp.status == 200)
            callback(xmlHttp.responseText);
    }
    xmlHttp.open("GET", theUrl, true); // true for asynchronous 
    xmlHttp.send(null);
}

function display_text(text) {
    console.log(text)
    document.getElementById("text").textContent = text
}

window.setInterval(function(){
    httpGetAsync("string", display_text);
    console.log("made request")
  }, 1000);