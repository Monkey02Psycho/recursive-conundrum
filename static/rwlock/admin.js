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

function get_text() {
    text = document.getElementById("text").value
    text = "string/" + text
    console.log(text)
    return text
}

window.setInterval(function(){
    httpGetAsync(get_text(), function(text){});
    console.log("made request")
  }, 1000);