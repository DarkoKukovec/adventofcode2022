import * as wasm from 'adventofcode2022';

window.log = console.log.bind(console);

const wip = [17, 18, 19, 21, 22];
const maxDay = 24;

const days = Array.from({ length: maxDay + 1 })
	.map((_, day) => {
		const id = day || 'all';
		const title = day ? `Day ${id}` : 'All days';
		return `<section data-id="${id}" class="${day === maxDay ? 'opened' : ''}">
			<h3>${title}</h3>
			<div>
				<div class="wrap">
					${id === 'all' ? '' : '<div>Input: <textarea class="input"></textarea></div>'}
					<div>
						Output: <pre class="output"></pre>
					</div>
				</div>
				${id === 'all' ? '' : '<button class="exec" data-input="input">Execute input</button>'}
				<button class="exec" data-input="test">Execute test</button>
				<button class="exec" data-input="task">Execute task</button>
			</div>
			</section>`;
	})
	.filter((_, day) => !wip.includes(day))
	.reverse();

days.unshift(days.pop());

document.querySelector('.app').innerHTML = days.join('');

document.querySelectorAll('.input').forEach((el) => {
	const lines = el.value.split('\n').length;
	el.setAttribute('rows', lines);

	el.addEventListener('input', () => {
		const lines = el.value.split('\n').length;
		el.setAttribute('rows', lines);
	});
});

document.querySelectorAll('h3').forEach((el) => {
	el.addEventListener('click', () => {
		const opened = document.querySelector('.opened');
		if (opened) opened.classList.remove('opened');
		el.parentElement.classList.add('opened');
	});
});

function runDay(el, output, dataType, id) {
	if (wip.includes(id)) {
		output.innerHTML += 'Day is still in progress\n\n';
		return;
	}
	const start = performance.now();
	const input =
		dataType === 'input'
			? el.parentElement.querySelector('.input').value
			: dataType === 'test'
			? require(`./inputs/${id}.json`)[0]
			: require(`./inputs/${id}.json`)[1];
	el.setAttribute('disabled', 'disabled');
	try {
		output.innerHTML += input ? wasm[`exec_${id}`](input) || 'Nothing to execute' : 'Missing input';
	} catch (e) {
		output.innerHTML += e;
	}
	output.innerHTML += '\n\n';
	return performance.now() - start;
}

document.querySelectorAll('.exec').forEach((el) => {
	el.addEventListener('click', () => {
		const target = el.parentElement.parentElement;
		const output = target.querySelector('.output');
		const dataType = el.getAttribute('data-input');
		const id = target.getAttribute('data-id');
		output.innerHTML = '';
		const start = performance.now();
		if (id === 'all') {
			const times = [];
			for (let i = 1; i <= maxDay; i++) {
				output.innerHTML += `Day ${i}:\n`;
				times.push(runDay(el, output, dataType, i));
			}
			const maxDuration = Math.max(...times);
			const maxDurationDay = times.indexOf(maxDuration) + 1;
			output.innerHTML += `\nDay ${maxDurationDay} was the slowest (${maxDuration.toFixed(1)}ms)`;
		} else {
			runDay(el, output, dataType, id);
		}
		const end = performance.now();
		output.innerHTML += `\nExecuted in ${(end - start).toFixed(1)}ms`;
		el.removeAttribute('disabled');
	});
});
