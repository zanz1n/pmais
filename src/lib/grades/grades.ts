export type ID = number;

export interface GradesData {
    registrations: Registration[];
    subjects: Subject[];
    globalSubjects: GlobalSubject[];
    periods: Period[];
}

export interface Registration {
    id: ID;
    year: number;
    name: string;
}

export interface BaseSubject {
    id: ID;
    name: string;
    shortName?: string;

    final: boolean;

    classAverage: string;
    classification: string;
    studentAverage: string;
    expression: string;
    color?: GradesColor;
}

export interface SingleFrontSubject extends BaseSubject {
    kind: SubjectKind.SingleFront;

    periods: SingleFrontPeriod[];
}

export interface MultipleFrontsSubject extends BaseSubject {
    kind: SubjectKind.MultipleFronts;
    periods: MultipleFrontsPeriod[];
    fronts: Front[];
}

export type Subject = SingleFrontSubject | MultipleFrontsSubject;

export interface BasePeriod {
    id: ID;
    periodId: ID;
    name: string;
    shortName?: string;

    classAverage: string;
    classification: string;
    studentAverage: string;
    expression: string;
    color?: GradesColor;
}

export interface SingleFrontPeriod extends BasePeriod {
    lastTest?: Test;
    tests: Test[];
}

export interface MultipleFrontsPeriod extends BasePeriod { }

export interface Front extends BaseSubject {
    periods: SingleFrontPeriod[];
}

export interface Test {
    id: ID;
    name: string;
    shortName?: string;

    grade: string;
    color: GradesColor;

    answers: Answer[];

    missed: boolean;
    substitute: boolean;
}

export interface Answer {
    number: number;
    student: string;
    correct: string;
}

export interface GlobalSubject {
    id: ID;
    name: string;

    students: number;
    studentsRegion: number;
    regionName: string;

    classAverage: string;
    classification: string;
    studentAverage: string;
    expression: string;
    color?: GradesColor;

    tests: Test[];
}

export interface Period {
    id: ID;
    name: string;
}

export enum GradesColor {
    Black = "BLACK",
    Green = "GREEN",
    Red = "RED",
}

export enum SubjectKind {
    SingleFront = "SINGLE_FRONT",
    MultipleFronts = "MULTIPLE_FRONTS",
}
