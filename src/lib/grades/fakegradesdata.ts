import { GradesColor, SubjectKind, type Front, type GradesData, type Subject } from "./grades";

const mat: Subject = {
    id: 1,
    name: "Matemática",
    final: false,
    classAverage: "5.5",
    classification: "14",
    studentAverage: "9.5",
    expression: "[P1]",
    color: GradesColor.Green,
    kind: SubjectKind.SingleFront,
    periods: [
        {
            id: 1,
            periodId: 1,
            name: "1° Trimestre",
            shortName: "D1",
            classAverage: "5,5",
            classification: "14",
            studentAverage: "5.9",
            expression: "[P1]",
            color: GradesColor.Red,
            tests: []
        },
        {
            id: 2,
            periodId: 2,
            name: "2° Trimestre",
            shortName: "D2",
            classAverage: "5,8",
            classification: "29",
            studentAverage: "6.5",
            expression: "[P1]",
            color: GradesColor.Green,
            tests: []
        },
        {
            id: 3,
            periodId: 3,
            name: "3° Trimestre",
            shortName: "D3",
            classAverage: "4.2",
            classification: "29",
            studentAverage: "7.2",
            expression: "[P1]",
            color: GradesColor.Green,
            tests: []
        }
    ],
} satisfies Subject;

const port: Subject = {
    id: 2,
    name: "Português",
    final: false,
    classAverage: "7.8",
    classification: "14",
    studentAverage: "7.5",
    expression: "[P1]",
    color: GradesColor.Green,
    kind: SubjectKind.SingleFront,
    periods: [
        {
            id: 1,
            periodId: 1,
            name: "1° Trimestre",
            shortName: "D1",
            classAverage: "5,5",
            classification: "14",
            studentAverage: "9.5",
            expression: "[P1]",
            color: GradesColor.Green,
            tests: []
        },
        {
            id: 2,
            periodId: 2,
            name: "2° Trimestre",
            shortName: "D2",
            classAverage: "5,8",
            classification: "29",
            studentAverage: "5.6",
            expression: "[P1]",
            color: GradesColor.Red,
            tests: []
        },
        {
            id: 3,
            periodId: 3,
            name: "3° Trimestre",
            shortName: "D3",
            classAverage: "4.2",
            classification: "29",
            studentAverage: "8.4",
            expression: "[P1]",
            color: GradesColor.Green,
            tests: []
        }
    ],
} satisfies Subject;

const final: Subject = {
    id: 0,
    name: "Nota final",
    final: true,
    classAverage: "7.5",
    classification: "56",
    studentAverage: "9.2",
    expression: "[D1]",
    color: GradesColor.Black,
    kind: SubjectKind.MultipleFronts,
    fronts: [
        mat as Front
    ],
    periods: [
        {
            id: 1,
            periodId: 1,
            name: "1° Trimestre",
            shortName: "D1",
            classAverage: "5,5",
            classification: "14",
            studentAverage: "9.5",
            expression: "[P1]",
        },
        {
            id: 2,
            periodId: 2,
            name: "2° Trimestre",
            shortName: "D2",
            classAverage: "5,8",
            classification: "29",
            studentAverage: "9.1",
            expression: "[P1]",
        },
        {
            id: 3,
            periodId: 3,
            name: "3° Trimestre",
            shortName: "D3",
            classAverage: "4.2",
            classification: "29",
            studentAverage: "8.4",
            expression: "[P1]",
        }
    ],
} satisfies Subject;

const subjects: Subject[] = [
    mat,
    port,
    mat,
    port,
    mat,
    port,
    final
] satisfies Subject[];

const gradesData: GradesData = {
    registrations: [
        {
            id: 1,
            name: "2024 - Segunda série",
            year: 2024,
        }
    ],
    subjects,
    globalSubjects: [],
    periods: [
        {
            id: 1,
            name: "1° Trimestre"
        },
        {
            id: 2,
            name: "2° Trimestre"
        },
        {
            id: 3,
            name: "3° Trimestre"
        }
    ],
} satisfies GradesData;

export default gradesData;
