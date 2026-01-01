document.addEventListener('DOMContentLoaded', () => {
    initStarfield();

    const calculateBtn = document.getElementById('calculate-btn');
    const targetDateInput = document.getElementById('target-date');
    const resultsArea = document.getElementById('results');
    
    const lyValue = document.getElementById('ly-value');
    const kmValue = document.getElementById('km-value');
    const miValue = document.getElementById('mi-value');
    const displayDate = document.getElementById('display-date');
    const displayLy = document.getElementById('display-ly');
    const voyagerTime = document.getElementById('voyager-time');

    const landmarkContainer = document.getElementById('landmark-container');
    const landmarkName = document.getElementById('landmark-name');
    const landmarkType = document.getElementById('landmark-type');
    const landmarkDesc = document.getElementById('landmark-desc');
    const landmarkDistVal = document.getElementById('landmark-dist-val');

    // Set default value to 10 years ago
    const now = new Date();
    const tenYearsAgo = new Date(now.getFullYear() - 10, now.getMonth(), now.getDate(), now.getHours(), now.getMinutes());
    targetDateInput.value = tenYearsAgo.toISOString().slice(0, 16);

    // Milestone buttons
    document.querySelectorAll('.milestone-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            targetDateInput.value = btn.dataset.date;
            calculateBtn.click();
        });
    });

    calculateBtn.addEventListener('click', async () => {
        const targetDate = targetDateInput.value;
        
        if (!targetDate) {
            alert('Please select a target date.');
            return;
        }

        // Convert to ISO string with timezone
        const isoDate = new Date(targetDate).toISOString();

        try {
            calculateBtn.disabled = true;
            calculateBtn.textContent = 'Calculating...';

            const response = await fetch('/api/calculate', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ target_date: isoDate }),
            });

            if (!response.ok) {
                const errorData = await response.json();
                throw new Error(errorData.error || 'Failed to calculate');
            }

            const data = await response.json();

            // Format numbers
            lyValue.textContent = data.light_years.toLocaleString(undefined, { minimumFractionDigits: 6, maximumFractionDigits: 6 });
            kmValue.textContent = Math.round(data.kilometers).toLocaleString();
            miValue.textContent = Math.round(data.miles).toLocaleString();
            voyagerTime.textContent = data.travel_time_voyager;
            
            displayDate.textContent = new Date(targetDate).toLocaleString();
            displayLy.textContent = data.light_years.toLocaleString(undefined, { maximumFractionDigits: 4 });

            // Update Landmark
            if (data.nearest_landmark) {
                landmarkName.textContent = data.nearest_landmark.name;
                landmarkType.textContent = data.nearest_landmark.object_type;
                landmarkDesc.textContent = data.nearest_landmark.description;
                landmarkDistVal.textContent = data.nearest_landmark.distance_ly.toLocaleString();
                landmarkContainer.classList.remove('hidden');
            } else {
                landmarkContainer.classList.add('hidden');
            }

            resultsArea.classList.remove('hidden');
            resultsArea.scrollIntoView({ behavior: 'smooth' });

        } catch (error) {
            alert(`Error: ${error.message}`);
        } finally {
            calculateBtn.disabled = false;
            calculateBtn.textContent = 'Focus Telescope';
        }
    });
});

function initStarfield() {
    const canvas = document.getElementById('starfield');
    const ctx = canvas.getContext('2d');
    
    let width, height;
    let stars = [];

    function resize() {
        width = window.innerWidth;
        height = window.innerHeight;
        canvas.width = width;
        canvas.height = height;
        createStars();
    }

    function createStars() {
        stars = [];
        const numStars = Math.floor((width * height) / 1000);
        for (let i = 0; i < numStars; i++) {
            stars.push({
                x: Math.random() * width,
                y: Math.random() * height,
                radius: Math.random() * 1.5,
                alpha: Math.random(),
                velocity: Math.random() * 0.05
            });
        }
    }

    function animate() {
        ctx.clearRect(0, 0, width, height);
        
        stars.forEach(star => {
            ctx.beginPath();
            ctx.arc(star.x, star.y, star.radius, 0, Math.PI * 2);
            ctx.fillStyle = `rgba(255, 255, 255, ${star.alpha})`;
            ctx.fill();

            // Simple parallax/movement
            star.y -= star.velocity;
            if (star.y < 0) {
                star.y = height;
                star.x = Math.random() * width;
            }
        });

        requestAnimationFrame(animate);
    }

    window.addEventListener('resize', resize);
    resize();
    animate();
}