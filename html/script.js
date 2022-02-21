function copyToClipboard() {
    var url = document.getElementById("link-to-redirect")
    var textArea = document.createElement("textarea");
    textArea.value = url.href;
    textArea.style.top = "0";
    textArea.style.left = "0";
    textArea.style.position = "fixed";

    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();
    document.execCommand("copy");

    document.body.removeChild(textArea);
}

async function shortenurl() {
    var redirect = document.getElementById("redirect").value;
    var password = document.getElementById("password").value;
    var data = {
        "redirect": redirect,
        "password": password
    }

    var request = await fetch(
        "/shorten",
        {
            method: "post",
            headers: {
                "Content-Type": "application/json;charset=utf-8"
            },
            body: JSON.stringify(data)
        }
    )
    var el = document.getElementById("link-to-redirect")
    if (request.ok) {
        var uri = (await request.json())["uri"]
        el.href = uri 
        el.innerText = uri
        el.parentElement.style.display = ""
    } else {
        el.innerText = "That is not a valid URL or password!"
        el.parentElement.style.display = ""
    }
}


// Make the DIV element draggable:
dragElement(document.getElementById("draggable-window"));

function dragElement(elmnt) {
    var pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
    if (document.getElementById(elmnt.id + "-header")) {
        // if present, the header is where you move the DIV from:
        document.getElementById(elmnt.id + "-header").onmousedown = dragMouseDown;
    } else {
        // otherwise, move the DIV from anywhere inside the DIV:
        elmnt.onmousedown = dragMouseDown;
    }

    function dragMouseDown(e) {
        e = e || window.event;
        e.preventDefault();
        // get the mouse cursor position at startup:
        var mx = e.clientX;
        var my = e.clientY;
        var rect = elmnt.getBoundingClientRect();
        offsetx = mx - rect.x
        offsety = my - rect.y

        document.onmouseup = closeDragElement;
        // call a function whenever the cursor moves:
        document.onmousemove = elementDrag;
    }

    function elementDrag(e) {
        e = e || window.event;
        e.preventDefault();

        var mx = e.clientX;
        var my = e.clientY;
        var w = window.innerWidth
        var h = window.innerHeight

        var rect = elmnt.getBoundingClientRect();
        
        var elx = (mx - offsetx + rect.width/2)
        var ely = (my - offsety + rect.height/2)

        elx = Math.max(Math.min(elx, w-rect.width/2), rect.width/2)
        ely = Math.max(Math.min(ely, h-rect.height/2), rect.height/2)

        elmnt.style.left = elx + "px";
        elmnt.style.top = ely + "px";

    }

    function closeDragElement() {
        // stop moving when mouse button is released:
        document.onmouseup = null;
        document.onmousemove = null;
    }
}

function windowStayOnPage() {
    var elmnt = document.getElementById("draggable-window")
    var w = window.innerWidth
    var h = window.innerHeight

    var rect = elmnt.getBoundingClientRect();

    elx = Math.max(Math.min(rect.x, w-rect.width), 0)
    ely = Math.max(Math.min(rect.y, h-rect.height), 0)

    elmnt.style.left = (elx + rect.width/2) + "px";
    elmnt.style.top = (ely + rect.height/2) + "px";
}
window.addEventListener ("resize", windowStayOnPage, true);