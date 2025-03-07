import Counter
import SwiftUI

@Observable class ViewModel: FfiUpdater {
    var rust: FfiApp
    var currentRoute: Route?
    var router: Router
    var navigationPath: [Route] = []

    public init() {
        let documentsPath = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
            .first!.absoluteString
        let rust = FfiApp(dataDir: documentsPath)

        self.rust = rust
        self.currentRoute = rust.getCurrentRoute()
        self.router = rust.getRouter()
        self.navigationPath = self.router.routes

        self.rust.listenForUpdates(updater: self)
    }

    func update(update: Update) {
        switch update {
        case .databaseUpdate:
            self.currentRoute = rust.getCurrentRoute()
            self.router = rust.getRouter()
            self.navigationPath = self.router.routes
            print("navigation path", self.navigationPath)
        }
    }

    // Helper function to get the current route
    func getCurrentRoute() -> Route {
        return currentRoute ?? .home
    }

    public func dispatch(event: Event) {
        self.rust.dispatch(event: event)
    }

    // Additional state
    var isSuccessScreenShown: Bool = false
}
