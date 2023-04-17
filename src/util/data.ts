function fillLabels(count: number): number[] {
    return new Array(count).fill(0).map((_, i) => ++i);
}

// NOTE(dylhack): The backend gives us C-like arrays
//                where -1 represents the end.
function getTillNull<T>(
    data: T[],
    entries: number,
    type: string
): [T[], string] {
    const result: T[] = [];

    for (let i = 0; i < data.length; i++) {
        const y = data[i];
        if (y === -1 || i + 1 >= entries) {
            break;
        }
        result.push(y);
    }

    return [result, type];
}

export function filterData<T>(
    data: GraphData<T>,
    filter: Filter
): [T[], string] {
    let filtered = [];
    let entries = 0;
    let type = "";

    switch (filter) {
        case "one_week":
            filtered = data.one_week;
            entries = 7;
            type = "day";
        case "one_month":
            entries = 30;
            filtered = data.one_month;
            type = "day";
        case "three_months":
            entries = 90;
            filtered = data.three_months;
            type = "day";
        case "six_months":
            entries = 180;
            filtered = data.six_months;
            type = "day";
        case "one_year":
            entries = 12;
            filtered = data.one_year;
            type = "month";
        case "three_years":
            entries = 36;
            filtered = data.three_years;
            type = "month";
    }

    return getTillNull(filtered, entries, type);
}

// NOTE(dylhack): please help me I haven't seen my family in 3 days
//                this guy is so obsessed with YouTube he doesn't even
//                know what a function is anymore
export function conaticus(
    filter: Filter,
    actual: GraphData<number>,
    predicted?: GraphData<number>
): ProjectedGraphData {
    const [filteredActual, type] = filterData<number>(actual, filter);
    const [filteredPredicted] = predicted
        ? filterData<number>(predicted, filter)
        : [];
    const label = type === "day" ? `Day %d` : `Month %d`;

    return {
        actual: filteredActual,
        predicted: filteredPredicted,
        labels: fillLabels(filteredActual.length).map((i) =>
            label.replace("%d", i.toString())
        ),
    };
}
