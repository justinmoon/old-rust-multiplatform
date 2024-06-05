import SwiftUI

@Observable class ViewModel: FfiUpdater {
    var count: Int32 = 0;
    var rust: FfiApp;
    
    public init() {
        self.rust = FfiApp()
        self.rust.listenForUpdates(updater: self)
    }
    
    func update(update: Update) {
        switch update {
        case .countChanged(count: let count):
            self.count = count
        }
    }
    
    public func dispatch(event: Event) {
        self.rust.dispatch(event: event)
    }
}

struct Counter: View {
    @State var rust: ViewModel;
    
    public init() {
        self.rust = ViewModel()
    }

    var body: some View {
        HStack {
            Button(action: {
                self.rust.dispatch(event: .decrement)
            }) {
                Text("-")
                    .font(.largeTitle)
                    .frame(width: 50, height: 50)
                    .background(Color.red)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }

            Text("\(self.rust.count)")
                .font(.largeTitle)
                .frame(width: 50, height: 50)

            Button(action: {
                self.rust.dispatch(event: .increment)
            }) {
                Text("+")
                    .font(.largeTitle)
                    .frame(width: 50, height: 50)
                    .background(Color.green)
                    .foregroundColor(.white)
                    .cornerRadius(10)
            }
        }
        .padding()
    }
}

struct Counter_Previews: PreviewProvider {
    static var previews: some View {
        Counter()
    }
}
    
