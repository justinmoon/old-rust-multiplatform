import SwiftUI

@Observable class ViewModel: FfiUpdater {
    var rust: FfiApp
    let db = Database()
    var count: String
    var router: Router

    public init() {
        let documentsPath = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
            .first!.absoluteString
        let rust = FfiApp(dataDir: documentsPath)
        let state = rust.getState()

        self.count = db.getCounter()
        self.router = state.router
        self.rust = rust

        self.rust.listenForUpdates(updater: self)

    }

    func update(update: Update) {
        switch update {
        case .databaseUpdate:
            self.count = db.getCounter()
        case .routerUpdate(let router):
            self.router = router
        }
    }

    public func dispatch(event: Event) {
        self.rust.dispatch(event: event)
    }
}
