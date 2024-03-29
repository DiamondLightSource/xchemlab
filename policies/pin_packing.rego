package xchemlab.pin_packing

import data.xchemlab
import future.keywords.if

default read_crystal = {"allowed": false}

default write_crystal = {"allowed": false}

default read_pin_library = {"allowed": false}

default write_pin_library = {"allowed": false}

default read_pin_mount := {"allowed": false}

default write_pin_mount := {"allowed": false}

default read_puck_library = {"allowed": false}

default write_puck_library = {"allowed": false}

default read_puck_mount := {"allowed": false}

default write_puck_mount = {"allowed": false}

default read_cane_library = {"allowed": false}

default write_cane_library = {"allowed": false}

default read_cane_mount := {"allowed": false}

default write_cane_mount := {"allowed": false}

read_crystal = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

write_crystal = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

read_pin_library = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

write_pin_library = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

read_pin_mount = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

write_pin_mount = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

read_puck_library = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

write_puck_library = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

read_puck_mount = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

write_puck_mount = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

read_cane_library = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

write_cane_library = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

read_cane_mount = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}

write_cane_mount = response if {
	xchemlab.valid_token
	response := {
		"allowed": true,
		"subject": xchemlab.subject,
	}
}
