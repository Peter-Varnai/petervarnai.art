
// Tab switching logic

const tabsAndSections = {
    'addProjectTab': [
        document.getElementById('addProjectTab'),
        document.getElementById('addProjectSection')],
    'editProjectTab': [
        document.getElementById('editProjectTab'),
        document.getElementById('editProjectSection')],
    'deleteProjectTab': [
        document.getElementById('deleteProjectTab'),
        document.getElementById('deleteProjectSection')],
    'addExhibitionTab': [
        document.getElementById('addExhibitionTab'),
        document.getElementById('addExhibitionSection')],
    'deleteExhibitionTab': [
        document.getElementById('deleteExhibitionTab'),
        document.getElementById('deleteExhibitionSection')],
}


document.querySelectorAll('.tab').forEach(button => {
    button.addEventListener('click', () => {
        handleTabClick(button.id);
    });
});


function handleTabClick(activeTab) {
    Object.entries(tabsAndSections).forEach(([key, value]) => {
        if (key === activeTab) {
            value[0].classList.add('active')
            value[1].classList.remove('hidden')
        } else {
            value[0].classList.remove('active')
            value[1].classList.add('hidden')
        }
    })
}



// LOGGING OUT
document.querySelectorAll('.logout-btn').forEach(btn => {
    btn.addEventListener('click', () => {
        fetch('/logout', {
            method: 'POST',
        }).then(response => {
            response.ok ?
                window.location.href = "/" :
                console.warn("error logging out")
        })
    })
})


document.querySelectorAll('.to-main-page').forEach(btn => {
    btn.addEventListener('click', () => window.location.href = '/')
})


function renderImageGallery(project) {
    const gallery = document.getElementById("image-gallery");
    gallery.innerHTML = ""; // clear old project images
    gallery.dataset.projectId = project.id;

    for (const filename of project.saved_files) {
        const item = document.createElement("div");
        item.className = "image-item";
        item.dataset.filename = filename;

        const img = document.createElement("img");
        img.src = `/f/static/images/${project.dir}/${filename}`;
        img.className = "thumbnail";
        img.alt = filename;

        const delBtn = document.createElement("button");
        delBtn.type = "button";
        delBtn.className = "delete-image-btn";
        delBtn.textContent = "Delete";

        item.appendChild(img);
        item.appendChild(delBtn);
        gallery.appendChild(item);
    }
}
