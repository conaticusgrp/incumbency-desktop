export type GraphData<T> = {
    // 0 = Daily, 1 = Monthly
    type_id: 0 | 1;
    one_week: T[];
    one_month: T[];
    three_months: T[];
    six_months: T[];
    one_year: T[];
    three_years: T[];
};

type GraphDataWithLabel<T> = {
  label: string;
} & GraphData<T>;

export enum Filter {
    OneWeek = "one_week",
    OneMonth = "one_month",
    ThreeMonths = "three_months",
    SixMonths = "six_months",
    OneYear = "one_year",
    ThreeYears = "three_years",
}

export type Data<T> = {
  actual: { label: string; data: T[]; };
  predicted?: { label: string; data: T[]; };
  type: GraphData<T>['type_id'];
  labels: number[] | string[];
}

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

function filterData<T>(data: GraphData<T>, filter: Filter): T[] {
  let filtered = [];
  switch (filter) {
      case Filter.OneWeek:
          filtered = data.one_week;
      case Filter.OneMonth:
          filtered = data.one_month;
      case Filter.ThreeMonths:
          filtered = data.three_months;
      case Filter.SixMonths:
          filtered = data.six_months;
      case Filter.OneYear:
          filtered = data.one_year;
      case Filter.ThreeYears:
          filtered = data.three_years;
  }

  return getTillNull(filtered);
}

// NOTE(dylhack): please help me I haven't seen my family in 3 days
//                this guy is so obsessed with YouTube he doesn't even 
//                know what a function is anymore
export function conaticus<T>(
  filter: Filter,
  actual: GraphDataWithLabel<T>,
  predicted?: GraphDataWithLabel<T>,
): Data<T> {
  const filteredActual = filterData<T>(actual, filter);
  const filteredPredicted = predicted ? filterData<T>(predicted, filter) : [];

  return {
    type: actual.type_id,
    actual: {
      label: actual.label,
      data: filteredActual,
    },
    predicted: predicted ? {
      label: predicted.label,
      data: filteredPredicted,
    } : undefined,
    labels: fillLabels(filteredActual.length).map(i => `Day ${i}`),
  }
}
