import type { StateCreator } from "zustand"
import { buildTimeBudgetPanelOpenState } from "./mainline-bridge-rules"
import type { MainlineShellState, ViewType } from "./types"

type UISet = Parameters<StateCreator<LayoutActionState>>[0]

type LayoutActionState = {
  activeView: ViewType
  leftSidebarExpanded: boolean
  rightDrawerOpen: boolean
  mobileMenuOpen: boolean
  mobileDrawerOpen: boolean
  mainlineExpanded: boolean
  evidencePanelOpen: boolean
  keyEvidencePanelOpen: boolean
  switchPanelOpen: boolean
  personalizedPanelOpen: boolean
  timeBudgetPanelOpen: boolean
  mainlineShell: MainlineShellState
}

export function createUILayoutActions(set: UISet) {
  return {
    setActiveView: (activeView: ViewType) => set({ activeView }),
    toggleLeftSidebar: () => set((state: LayoutActionState) => ({ leftSidebarExpanded: !state.leftSidebarExpanded })),
    toggleRightDrawer: () => set((state: LayoutActionState) => ({ rightDrawerOpen: !state.rightDrawerOpen })),
    setRightDrawerOpen: (rightDrawerOpen: boolean) => set({ rightDrawerOpen }),
    setMobileMenuOpen: (mobileMenuOpen: boolean) => set({ mobileMenuOpen }),
    setMobileDrawerOpen: (mobileDrawerOpen: boolean) => set({ mobileDrawerOpen }),
    setMainlineExpanded: (mainlineExpanded: boolean) => set({ mainlineExpanded }),
    setEvidencePanelOpen: (evidencePanelOpen: boolean) => set({ evidencePanelOpen }),
    setKeyEvidencePanelOpen: (keyEvidencePanelOpen: boolean) => set({ keyEvidencePanelOpen }),
    setSwitchPanelOpen: (switchPanelOpen: boolean) => set({ switchPanelOpen }),
    setPersonalizedPanelOpen: (personalizedPanelOpen: boolean) => set({ personalizedPanelOpen }),
    setTimeBudgetPanelOpen: (timeBudgetPanelOpen: boolean) =>
      set((state: LayoutActionState) => buildTimeBudgetPanelOpenState(state.mainlineShell, timeBudgetPanelOpen)),
  }
}
