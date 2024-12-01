
# Prefix used to distinguish commentary created by this makefile
zA24M_THIS_MAKEFILE = a24m-console.mk

zA24M_TOOLS_DIR     = Tools

#########################
# Makefile Bash Console
#
# This is a sub makefile that contains several canned basic
# macros that support regular console interactivity.
#

zA24M_MBC_MAKEFILE = $(zA24M_TOOLS_DIR)/mbc.MakefileBashConsole.mk


# What console tool will put in prefix of each line
MBC_ARG__CONTEXT_STRING = $(zA24M_THIS_MAKEFILE)

include $(zA24M_MBC_MAKEFILE)

zA24M_START = $(MBC_SHOW_WHITE) "Rule $@: starting..."
zA24M_STEP  = $(MBC_SHOW_WHITE) "Rule $@:"
zA24M_PASS  = $(MBC_PASS)       "Rule $@: no errors."


# Use this to allow a console make target to explicitly trigger other
#   console make targets.  This is really only for making sure entire
#   rules function in 'big test cases': for efficiency, better to use
#   explicit fine grained make dependencies so that make is efficient.
zA24M_MAKE = $(MAKE) -f $(zA24M_THIS_MAKEFILE)

default:
	$(MBC_SHOW_RED) "NO TARGET SPECIFIED.  Check" $(zA24M_TABTARGET_DIR) "directory for options." && $(MBC_FAIL)

# OUCH scrub this out eventually
$(info A24_PARAMETER_0: $(A24_PARAMETER_0))
$(info A24_PARAMETER_1: $(A24_PARAMETER_1))
$(info A24_PARAMETER_2: $(A24_PARAMETER_2))
$(info A24_PARAMETER_3: $(A24_PARAMETER_3))



#######################################
#  Config Regime Info
#
#  This section has examples of several sample nameplates, some
#  of which are useful in maintaining this example space.


rbm-Ci.ConfigRegimeInfo.sh: rbs_define
	$(zA24M_PASS)



#######################################
#  Nameplate Examples
#
#  This section has examples of several sample nameplates, some
#  of which are useful in maintaining this example space.

pyghm-B.BuildPythonGithubMaintenance.sh:
	$(zA24M_STEP) "Assure podman services available..."
	which podman
	podman machine start || echo "Podman probably running already, lets go on..."
	$(zA24M_RBM_SUBMAKE) rbm-BL.sh  RBM_ARG_MONIKER=pyghm

pyghm-s.StartPythonGithubMaintenance.sh:
	$(zA24M_STEP) "Assure podman services available..."
	which podman
	podman machine start || echo "Podman probably running already, lets go on..."
	$(zA24M_RBM_SUBMAKE) rbm-s.sh  RBM_ARG_MONIKER=pyghm


# OUCH is this the right place?
oga.OpenGithubAction.sh:
	$(zA24M_STEP) "Assure podman services available..."
	cygstart https://github.com/bhyslop/recipemuster/actions/


#######################################
#  Tabtarget Maintenance Tabtarget
#
#  Helps you create default form tabtargets in right place.

# Location for tabtargets relative to top level project directory
zA24M_TABTARGET_DIR  = ./tt

# Parameter from the tabtarget: what is the full name of the new tabtarget
A24M_TABTARGET_NAME = 

zA24M_TABTARGET_FILE = $(zA24M_TABTARGET_DIR)/$(A24M_TABTARGET_NAME)

ttc.CreateTabtarget.sh:
	@test -n "$(A24M_TABTARGET_NAME)" || { echo "Error: missing name param"; exit 1; }
	@echo '#!/bin/sh' >         $(zA24M_TABTARGET_FILE)
	@echo 'cd "$$(dirname "$$0")/.." &&  Tools/tabtarget-dispatch.sh 1 "$$(basename "$$0")"' \
	                 >>         $(zA24M_TABTARGET_FILE)
	@chmod +x                   $(zA24M_TABTARGET_FILE)
	git add                     $(zA24M_TABTARGET_FILE)
	git update-index --chmod=+x $(zA24M_TABTARGET_FILE)
	$(zA24M_PASS)


ttx.FixTabtargetExecutability.sh:
	git update-index --chmod=+x $(zA24M_TABTARGET_DIR)/*
	$(zA24M_PASS)


#######################################
#  Slickedit Project Tabtarget
#
#  Due to filesystem handle entanglements, Slickedit doesn't play well
#  with git.  This tabtarget places a usable copy in a .gitignored location

zA24M_SLICKEDIT_PROJECT_DIR = ./_slickedit

vsr.ReplaceSlickEditWorkspace.sh:
	mkdir -p                                           $(zA24M_SLICKEDIT_PROJECT_DIR)
	-rm -rf                                            $(zA24M_SLICKEDIT_PROJECT_DIR)/*
	cp $(zA24M_TOOLS_DIR)/vsep_VisualSlickEditProject/* $(zA24M_SLICKEDIT_PROJECT_DIR)
	$(zA24M_PASS)

A24M_PARAM_DIR = 


#######################################
#  AOC 2024 stuff
#


A24M_RUST_IMAGE = ghcr.io/bhyslop/recipemuster:bottle_rust.20241201__213634


a24br%:
	@echo 'Display parameters... A24_PARAMETER_2=' $(A24_PARAMETER_2)
	@echo 'Pull rust image...'
	podman pull $(A24M_RUST_IMAGE)
	@echo 'Building initial rust app...'
	podman run -v ./:/app:Z -w /app/rust $(A24M_RUST_IMAGE) rustc $(A24_PARAMETER_2).rs
	@echo 'List initial rust app...'
	podman run -v ./:/app:Z -w /app/rust $(A24M_RUST_IMAGE) ls
	@echo 'Run initial rust app...'
	podman run -v ./:/app:Z -w /app/rust $(A24M_RUST_IMAGE) ./$(A24_PARAMETER_2)
	@echo 'done.'



# EOF
