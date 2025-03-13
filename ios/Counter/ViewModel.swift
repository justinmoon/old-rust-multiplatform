import Counter
import SwiftUI

@Observable class ViewModel: RmpViewModel {
    var model: RmpModel
    var count: Int32

    public init() {
        let documentsPath = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
            .first!.absoluteString
        let model = RmpModel(dataDir: documentsPath)
        
        self.model = model
        self.count = model.getCount();
        
        self.model.listenForModelUpdates(updater: self)
    }

    func modelUpdate(modelUpdate: ModelUpdate) {
        switch modelUpdate {
        case .countChanged(let count):
            self.count = count
        }
    }

    public func action(action: Action) {
        self.model.action(action: action)
    }
}
