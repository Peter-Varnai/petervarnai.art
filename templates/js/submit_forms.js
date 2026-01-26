// exhibitions related forms
document.getElementById('exhibitionForm').addEventListener('submit', async (e) => {
    e.preventDefault();

    const formData = {
        title: e.target.name.value,
        start_date: e.target.start_date.value,
        till: e.target.till.value,
        location: e.target.location.value,
        link: e.target.link.value,
        big_row: e.target.big_row.checked,
    };

    const response = await fetch('/exhibition', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(formData)
    }).then(response => {
        if (response.ok) {
            console.log('exhibition added succesfullly')
            window.location.reload()
        } else console.error('Error:', error);
    })


});

document.querySelectorAll(".delete-exhib-btn").forEach(btn => {
    btn.addEventListener("click", () => {
        console.log('del button clicked')
        let json_id = JSON.stringify({ 'id': Number(btn.dataset.id) })
        console.log('sending delete project request :', json_id)
        fetch('/exhibition', {
            method: 'DELETE',
            headers: { 'Content-Type': 'application/json' },
            body: json_id
        }).then(r => {
            if (r.redirected) window.location = r.url;
            else location.reload();
        });

    });
});



// projects related forms
document.getElementById('addProjectForm').addEventListener('submit', function(e) {
    e.preventDefault();
    const form = e.target;
    const formData = new FormData(form);

    console.log(formData)
    fetch('/project', {
        method: 'POST',
        body: formData,
    })
        .then(response => {
            if (response.ok) {
                console.log("succesful form submission", response)
                window.location.reload()
            } else {
                console.log("error while submitting form", response)
            }
        })
        .catch(error => {
            console.error('Error:', error);
        });
});

document.getElementById('editProjectForm').addEventListener('submit', function(e) {
    e.preventDefault();
    const form = e.target;
    const formData = new FormData(form);

    fetch('/project', {
        method: 'PUT',
        body: formData,
    }).then(response => {
        if (response.ok) {
            console.log("succesful form submission", response)
        } else {
            console.log("error while submitting form", response)
        }
    })
        .catch(error => {
            console.error('error:', error);
        });
});

document.querySelectorAll('.delete-project-btn').forEach(btn => {
    btn.addEventListener('click', function() {
        let json = JSON.stringify({
            'id': Number(btn.dataset.id),
            'folder_path': btn.dataset.folder,
        })
        console.log('sending request to delete project: ', json)
        fetch('/project', {
            method: 'DELETE',
            headers: { 'Content-Type': 'application/json' },
            body: json
        }).then(r => {
            console.log('answer :', r)
            if (r.ok) {
                window.location.reload();
            } else console.log('error deleting project', json);
        });
    })
})

// SELECT WHICH PROJECT TO EDIT
document.querySelectorAll('.editProjectBtn').forEach(btn => {
    btn.addEventListener('click', function() {
        console.log('edit project button pressed', btn.dataset.id)
        fetch(`/project?no=${btn.dataset.id}`)
            .then(r => r.json())
            .then(project => {
                const form = document.getElementById('editProjectForm');
                form.querySelector('input[name="title"]').value = project.title;
                form.querySelector('input[name="date"]').value = project.date.substring(0, 7); // YYYY-MM
                form.querySelector('input[name="video_link"]').value = project.video_link || '';
                form.querySelector('input[name="medium"]').value = project.medium || '';
                form.querySelector('input[name="duration"]').value = project.duration || '';
                form.querySelector('textarea[name="concept"]').value = project.concept;
                form.querySelector('input[name="dir"]').value = project.dir;
                form.querySelector('input[name="id"]').value = project.id;
                renderImageGallery(project);
                console.log('Form populated with:', project);
            })
            .catch(err => console.log('Error:', err));
    })
})

function renderImageGallery(project) {
    console.log(project)
    const gallery = document.getElementById("image-gallery");
    gallery.innerHTML = "";
    gallery.dataset.projectId = project.dir;
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

// PICTURE UPLOAD 
document.getElementById("upload-images-btn").addEventListener("click", async () => {
    const gallery = document.getElementById("image-gallery");
    const projectDir = gallery.dataset.projectId;

    if (!projectDir) {
        alert("No project selected.");
        return;
    }

    const input = document.getElementById("image-upload-input");
    if (!input.files.length) {
        alert("Please select at least one image.");
        return;
    }

    const formData = new FormData();
    for (const file of input.files) {
        formData.append("images", file);
    }

    try {
        const res = await fetch(`/projects/pic_update/${projectDir}/images`, {
            method: "POST",
            body: formData
        });

        if (!res.ok) throw new Error();

        const data = await res.json(); // 👈 read response
        console.log(data);

        renderImageGallery({
            dir: projectDir,
            saved_files: data.saved_files
        });

        input.value = "";

    } catch {
        alert("Image upload failed.");
    }
});



document.getElementById("image-gallery").addEventListener("click", async (e) => {
    if (!e.target.classList.contains("delete-image-btn")) return;

    const imageItem = e.target.closest(".image-item");
    const gallery = e.target.closest("#image-gallery");

    const filename = imageItem.dataset.filename;
    const dir = gallery.dataset.projectId;

    try {
        const res = await fetch(`/projects/pic_update/${dir}/images`, {
            method: "DELETE",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ filename })
        });

        if (!res.ok) throw new Error();

        const { saved_files } = await res.json();
        saved_files.dir = dir
        renderImageGallery({ dir, saved_files });

    } catch {
        alert("Failed to delete image.");
    }
});

