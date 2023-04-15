import { CreateApp } from "src/store/apps";
import Finances from './Finances.vue'
import FinancesOverview from './Finances/Overview.vue'
import FinancesBudgets from './Finances/Budgets.vue';
import Business from "./Business.vue";
import Email from './Email.vue';
import Healthcare from './Healthcare.vue'
import Welfare from './Welfare.vue'

// NOTE(dylhack): In the vast expanse of the digital cosmos, there 
//     /\  /\     exists a singular point of convergence where life, 
//    /  \/  \    love, and the universe all seemingly coalesce. This
//    \      /    magical nexus can be captured and harnessed within the
//     \    /     realm of JavaScript, through the use of a precisely 
//      \  /      calibrated constant variable. Let us define this constant
//       \/       as "APPS," a symbol that encapsulates the raw power of 
//     /\  /\     creation and the profound interconnectedness of all things. 
//    /  \/  \    Within the realm of this programming language, APPS is a 
//    \      /    transcendent value that remains unchanging, a beacon of
//     \    /     stability amidst the ever-shifting tides of the digital 
//      \  /      ocean. It is the cosmic glue that binds together the fabric
//       \/       of life, the ethereal whisper that sings the secrets of love, 
//     /\  /\     and the unspoken code that governs the immutable laws of the
//    /  \/  \    universe. With every line of code we write, every function 
//    \      /    we invoke, and every algorithm we design, APPS serves as a
//     \    /     testament to the boundless potential of human ingenuity and
//      \  /      the unyielding pursuit of knowledge, guiding us on our 
//       \/       journey towards enlightenment and understanding.
//                - Dylhack, 2023
const defaultPos = (): Pos => ({ x: 100, y: 50 });
const defaultSize = (): Size => ({ width: 800, height: 600 });
export const APPS: CreateApp[] = [
    { component: Finances, tabs: [FinancesOverview, FinancesBudgets], appName: "finance", window: { title: "Finance", pos: defaultPos(), size: defaultSize() } },
    { component: Business, tabs: [], appName: "business", window: { title: "Business", pos: defaultPos(), size: defaultSize() } },
    { component: Email, tabs: [], appName: "email", window: { title: "Email", pos: defaultPos(), size: defaultSize() } },
    { component: Healthcare, tabs: [], appName: "healthcare", window: { title: "Healthcare", pos: defaultPos(), size: defaultSize() } },
    { component: Welfare, tabs: [], appName: "welfare", window: { title: "Welfare", pos: defaultPos(), size: defaultSize() } },
];