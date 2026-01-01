document.addEventListener('DOMContentLoaded', () => {
    initStarfield();

    const calculateBtn = document.getElementById('calculate-btn');
    
    const inputYear = document.getElementById('input-year');
    const inputMonth = document.getElementById('input-month');
    const inputDay = document.getElementById('input-day');
    const inputTime = document.getElementById('input-time');

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

    const now = new Date();
    inputYear.value = now.getFullYear() - 10;
    inputMonth.value = now.getMonth() + 1;
    inputDay.value = now.getDate();
    inputTime.value = now.toTimeString().slice(0, 5);

    document.querySelectorAll('.milestone-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            const data = JSON.parse(btn.dataset.json);
            
            inputYear.value = data.year;
            inputMonth.value = data.month;
            inputDay.value = data.day;
            inputTime.value = data.time;

            calculateBtn.click();
        });
    });

    calculateBtn.addEventListener('click', async () => {
        const year = parseInt(inputYear.value);
        const month = parseInt(inputMonth.value);
        const day = parseInt(inputDay.value);
        const timeStr = inputTime.value || "12:00";
        const [hour, minute] = timeStr.split(':').map(Number);

        if (isNaN(year) || isNaN(month) || isNaN(day)) {
            alert('Please enter a valid date.');
            return;
        }

        try {
            calculateBtn.disabled = true;
            calculateBtn.textContent = 'Calculating...';

            const payload = {
                year: year,
                month: month,
                day: day,
                hour: hour || 0,
                minute: minute || 0
            };

            const response = await fetch('/api/calculate', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(payload),
            });

            if (!response.ok) {
                const errorData = await response.json();
                throw new Error(errorData.error || 'Failed to calculate');
            }

            const data = await response.json();

            lyValue.textContent = data.light_years.toLocaleString(undefined, { minimumFractionDigits: 6, maximumFractionDigits: 6 });
            kmValue.textContent = Math.round(data.kilometers).toLocaleString();
            miValue.textContent = Math.round(data.miles).toLocaleString();
            voyagerTime.textContent = data.travel_time_voyager;
            
            const era = year < 0 ? 'BCE' : 'CE';
            const absYear = Math.abs(year);
            const monthName = inputMonth.options[inputMonth.selectedIndex].text;
            displayDate.textContent = `${monthName} ${day}, ${absYear} ${era} at ${timeStr}`;
            
            displayLy.textContent = data.light_years.toLocaleString(undefined, { maximumFractionDigits: 4 });

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
