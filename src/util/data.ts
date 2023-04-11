function fillLabels(count: number): number[] {
  return new Array(count).fill(0).map((_, i) => ++i);
}

// NOTE(dylhack): The backend gives us C-like arrays 
//                where -1 represents the end.
function getTillNull<T>(data: T[]): T[] {
  const result: T[] = [];

  for (let i = 0; i < data.length; i++) {
    const y = data[i];
    if (y === -1) {
      break;
    }
    result.push(y);
  }

  return result;
}

export function filterData<T>(data: GraphData<T>, filter: Filter): T[] {
  let filtered = [];
  switch (filter) {
      case 'one_week':
          filtered = data.one_week;
      case 'one_month':
          filtered = data.one_month;
      case 'three_months':
          filtered = data.three_months;
      case 'six_months':
          filtered = data.six_months;
      case 'one_year':
          filtered = data.one_year;
      case 'three_years':
          filtered = data.three_years;
  }

  return getTillNull(filtered);
}

// NOTE(dylhack): please help me I haven't seen my family in 3 days
//                this guy is so obsessed with YouTube he doesn't even 
//                know what a function is anymore
export function conaticus(
  filter: Filter,
  actual: GraphData<number>,
  predicted?: GraphData<number>,
): ProjectedGraphData {
  const filteredActual = filterData<number>(actual, filter);
  const filteredPredicted = predicted ? filterData<number>(predicted, filter) : [];
  const label = actual.type_id === 0 ? `Day %d` : `Month %d`;

  return {
    actual: filteredActual,
    predicted: filteredPredicted,
    labels: fillLabels(filteredActual.length).map(i => label.replace("%d", i.toString())),
  }
}
