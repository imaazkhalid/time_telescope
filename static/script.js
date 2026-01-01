document.addEventListener('DOMContentLoaded', () => {
    const calculateBtn = document.getElementById('calculate-btn');
    const targetDateInput = document.getElementById('target-date');
    const resultsArea = document.getElementById('results');
    
    const lyValue = document.getElementById('ly-value');
    const kmValue = document.getElementById('km-value');
    const miValue = document.getElementById('mi-value');
    const displayDate = document.getElementById('display-date');
    const displayLy = document.getElementById('display-ly');

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
            
            displayDate.textContent = new Date(targetDate).toLocaleString();
            displayLy.textContent = data.light_years.toLocaleString(undefined, { maximumFractionDigits: 4 });

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
