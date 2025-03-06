import SwiftUI

@Observable class ViewModel: FfiUpdater {
    var rust: FfiApp
    var count: Int32
    var timer: TimerState
    var router: Router

    public init() {
        let documentsPath = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
            .first!.absoluteString;
        let rust = FfiApp(dataDir: documentsPath)
        let state = rust.getState()

        self.count = state.count
        self.timer = state.timer
        self.router = state.router
        self.rust = rust

        self.rust.listenForUpdates(updater: self)

    }

    func update(update: Update) {
        switch update {
        case .databaseUpdate:
            print("TODO: fetch state")
        case .timer(state: let timer):
            self.timer = timer
        case .routerUpdate(let router):
            self.router = router
        }
    }

    public func dispatch(event: Event) {
        self.rust.dispatch(event: event)
    }
}
