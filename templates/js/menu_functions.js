function collaboratorLinks(text, targetWord, link) {
    const regex = new RegExp(targetWord, "gi");
    return text.replace(regex, '<a class="collaborator-link" target="_blank" href=' + link + '>' + targetWord + '</a>');
}


function worksSubmenuOnClick(projectName) {
    toggleWorksWindow()
    loadProjectInfo(projectName)
}


async function loadProjectInfo(projectName) {
    // console.log(`/prj?no=${projectName}`)
    const response = await fetch(`/prj?no=${projectName}`)
    const html = await response.text()
    console.log(response)

    const projectView = document.getElementById('project-view')
    // console.log(html)
    projectView.innerHTML = html
}


function toggleWorksWindow() {
    const canvasTrgt = document.getElementById('3d-scene')
    canvasTrgt.style.filter = 'blur(18px)'
    const worksWindow = document.getElementById('works-window')
    const menuPoints = document.getElementById('menu-points')
    const peterTitle = document.getElementById('peter-title-mobile')
    const container = document.querySelector(".container")

    if (worksWindow.style.display === 'none') {
        canvasTrgt.style.filter = 'blur(18px)'

        container.style.cssText += `
            justify-content: center;
            align-items: center;
            `
        peterTitle.style.color = 'white'
        worksWindow.style.display = 'flex'
        menuPoints.style.display = 'none'
    } else {
        canvasTrgt.style.filter = 'blur(0px)'

        container.style.removeProperty('justify-content')
        container.style.removeProperty('align-items')
        document.getElementById('images-panel').innerHTML = ''
        document.getElementById('details-table').innerHTML = ''
        document.getElementById('concept-panel').innerHTML = ''
        peterTitle.style.color = 'black'
        worksWindow.style.display = 'none'
        menuPoints.style.display = 'block'
    }
}


function toggleAboutWindow() {

    const aboutWindow = document.getElementById('about-window')
    const menuPoints = document.getElementById('menu-points')
    const peterTitle = document.getElementById('peter-title-mobile')
    const container = document.querySelector(".container")

    if (aboutWindow.style.display === 'none') {
        container.style.cssText += `
            justify-content: center;
            align-items: center;
            `
        peterTitle.style.color = 'white'
        aboutWindow.style.display = 'flex'
        menuPoints.style.display = 'none'
    } else {
        container.style.removeProperty('justify-content')
        container.style.removeProperty('align-items')

        peterTitle.style.color = 'black'
        aboutWindow.style.display = 'none'
        menuPoints.style.display = 'block'
    }
}


function changePrimaryImage(clickedImage) {
    let primaryImage = document.querySelector('.primary-image')

    let clickedImageSrc = primaryImage.src
    primaryImage.src = clickedImage.src
    clickedImage.src = clickedImageSrc
}

