// SELECT WHICH PROJECT TO EDIT
document.querySelectorAll('.editProjectBtn').forEach(btn => {
    btn.addEventListener('click', function() {
        fetch(`/project?no=${btn.dataset.id}`)
            .then(r => r.json())
            .then(project => {
                const form = document.getElementById('editProjectForm');
                form.querySelector('input[name="title"]').value = project.title;
                form.querySelector('input[name="date"]').value = project.date.substring(0, 7); // YYYY-MM
                form.querySelector('input[name="video_link"]').value = project.video_link || '';
                form.querySelector('input[name="medium"]').value = project.medium || '';
                form.querySelector('input[name="duration"]').value = project.duration || '';
                window.editConceptQuill.root.innerHTML = project.concept || '';
                form.querySelector('input[name="dir"]').value = project.dir;
                form.querySelector('input[name="id"]').value = project.id;
                renderImageGallery(project);
            })
            .catch(err => console.log('Error:', err));
    })
})

function renderImageGallery(project) {
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

        const data = await res.json();

        renderImageGallery({
            dir: projectDir,
            saved_files: data.saved_files
        });

        input.value = "";

        console.log("succesfully uploaded picture/s")

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
        console.log("succesfully deleted project")

    } catch {
        alert("Failed to delete image.");
    }
});

