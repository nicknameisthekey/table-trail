export interface QueryResultData {
	columns: string[];
	rows: string[][];
}

export interface ContextMenuState {
	X: number;
	Y: number;
	Opened: boolean;
	Options: ContextMenuItem[];
}

export interface ContextMenuItem {
	text: string;
	callback: () => void;
}
