<script lang="ts">
	const today = new Date();
	let viewingMonth: number = today.getMonth();
	let viewingYear: number = today.getFullYear();

	function getDaysInMonth(): number {
		 	const daysInMonth = new Date(viewingYear, viewingMonth, 0).getDate();
			return daysInMonth;
	}

	let dayOfWeek = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
</script>

<div class="calendar-page">
	<div class="calendar-content">
		<div class="calendar-overview">
		</div>
		<div class="calendar">
			<div class="calendar-dates">
				{#each dayOfWeek as day (day)}
					<div class="calendar-day-name">
						{day}
					</div>
				{/each}
				{#each Array(35) as _, index (index)}
					{@const daysInMonth = getDaysInMonth()}
					{@const isToday = viewingMonth=== today.getMonth() && index === today.getDate() - 1 && viewingYear === today.getFullYear() ? "true" : "false"}
					<div
						class="calendar-date"
						data-next-month={index >= daysInMonth}
						data-today={isToday}
					>
						<span class="calendar-date-number" data-today={isToday}>
							{index % daysInMonth + 1}
						</span>
					</div>
				{/each}
			</div>
		</div>
	</div>
</div>

<style>
	.calendar-page {
			display: flex;
			align-items: center;
			justify-content: center;
			width: 100%;
			height: 100%;
	}
	.calendar-content {
			display: flex;
			width: 90%;
			height: 96%;
			gap: 12px;
	}
	.calendar-overview {
			width: 30%;
			height: 100%;
			border: 1px solid #ececec;
			border-radius: 16px;
	}
	.calendar {
			display: flex;
			width: 100%;
			height: 100%;
      border-radius: 16px;
			border: 1px solid #dfdfdf;
  }
	.calendar-dates {
			display: grid;
			grid-template-columns: repeat(7, 1fr);
			grid-template-rows: 30px repeat(5, 1fr);
			gap: 1px;
			border-radius: 16px;
			height: 100%;
			width: 100%;
	}
	.calendar-day-name {
			font-size: 12px;
			color: #616161;
			height: 32px;
			display: flex;
			align-items: center;
			justify-content: center;
			font-weight: 600;
	}
	.calendar-day-name:first-child {
			border-top-left-radius: inherit;
	}
	.calendar-day-name:nth-child(7) {
      border-top-right-radius: inherit;
  }
	.calendar-date {
			display: flex;
			flex-direction: column;
			width: 100%;
			height: 100%;
			border-top: 1px solid #ececec;
			border-right: 1px solid #ececec;
	}
	.calendar-date:last-child {
			border-bottom-right-radius: inherit;
	}
	.calendar-date[data-next-month="true"] {
			background-color: #d5d5d5 !important;
	}
  .calendar-date[data-next-month="true"] > .calendar-date-number {
      color: #9e9e9e !important;
  }
	.calendar-date-number {
			margin-top: 14px;
			margin-left: 14px;
			font-size: 10px;
			font-weight: 600;
			color: #616161;
			aspect-ratio: 1/1;
			width: 14px;
      display: flex;
      align-items: center;
      justify-content: center;
	}
	.calendar-date-number[data-today="true"] {
			background-color: var(--primary);
			color: white;
			padding: 2px;
			border-radius: 50%;
	}
</style>