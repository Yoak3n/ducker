export interface Action {
    id: string;
    name: string;
    description: string;
    type: string
    wait: number;
    cmd: string;
    args: string[];
}